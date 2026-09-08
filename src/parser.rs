#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    TopToBottom,
    BottomToTop,
    LeftToRight,
    RightToLeft,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeShape {
    Rectangle,  // [label]
    Rounded,    // (label)
    Diamond,    // {label}
    Cylinder,   // [(label)]
    Circle,     // ((label))
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeStyle {
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub shape: NodeShape,
    pub style: Option<NodeStyle>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EdgeType {
    SolidArrow,   // -->
    DottedArrow,  // -.->
    ThickArrow,   // ==>
    SolidLine,    // ---
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone)]
pub struct Flowchart {
    pub direction: Direction,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone)]
pub struct SequenceMessage {
    pub from: String,
    pub to: String,
    pub label: String,
    pub arrow: String,
}

#[derive(Debug, Clone)]
pub enum SequenceItem {
    Message(SequenceMessage),
    Note { over: Vec<String>, text: String },
    LoopStart { label: String },
    LoopEnd,
    AltStart { label: String },
    AltElse { label: String },
    AltEnd,
}

#[derive(Debug, Clone)]
pub struct SequenceDiagram {
    pub participants: Vec<String>,
    pub items: Vec<SequenceItem>,
}

#[derive(Debug, Clone)]
pub struct ClassMember {
    pub visibility: char,
    pub name: String,
    pub member_type: String,
    pub is_method: bool,
}

#[derive(Debug, Clone)]
pub struct ClassItem {
    pub name: String,
    pub members: Vec<ClassMember>,
}

#[derive(Debug, Clone)]
pub struct ClassRelation {
    pub from: String,
    pub to: String,
    pub relation_type: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ClassDiagram {
    pub classes: Vec<ClassItem>,
    pub relations: Vec<ClassRelation>,
}

#[derive(Debug, Clone)]
pub enum ParsedDiagram {
    Flowchart(Flowchart),
    Sequence(SequenceDiagram),
    Class(ClassDiagram),
}

pub fn parse_mermaid(input: &str) -> Result<ParsedDiagram, String> {
    let lines: Vec<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with("%%"))
        .collect();

    if lines.is_empty() {
        return Err("Empty input".to_string());
    }

    let header = lines[0].to_lowercase();
    if header.starts_with("graph") || header.starts_with("flowchart") {
        parse_flowchart(&lines)
    } else if header.starts_with("sequencediagram") {
        parse_sequence(&lines)
    } else if header.starts_with("classdiagram") {
        parse_class_diagram(&lines)
    } else {
        Err(format!("Unsupported diagram header: {}", lines[0]))
    }
}

fn parse_flowchart(lines: &[&str]) -> Result<ParsedDiagram, String> {
    let header_parts: Vec<&str> = lines[0].split_whitespace().collect();
    let direction = if header_parts.len() > 1 {
        match header_parts[1].to_uppercase().as_str() {
            "LR" => Direction::LeftToRight,
            "RL" => Direction::RightToLeft,
            "BT" => Direction::BottomToTop,
            _ => Direction::TopToBottom,
        }
    } else {
        Direction::TopToBottom
    };

    let mut nodes = std::collections::HashMap::new();
    let mut edges = Vec::new();
    let mut styles: std::collections::HashMap<String, NodeStyle> = std::collections::HashMap::new();

    for line in &lines[1..] {
        let trimmed = line.trim();

        // Check for style directives: style NodeId fill:#f9f,stroke:#333,stroke-width:4px,color:#fff
        if trimmed.starts_with("style ") {
            let parts: Vec<&str> = trimmed.trim_start_matches("style ").split_whitespace().collect();
            if parts.len() >= 2 {
                let node_id = parts[0].trim();
                let style_str = parts[1..].join(" ");
                let mut node_style = NodeStyle { fill: None, stroke: None, color: None };

                for prop in style_str.split(',') {
                    let kv: Vec<&str> = prop.split(':').collect();
                    if kv.len() == 2 {
                        let k = kv[0].trim();
                        let v = kv[1].trim();
                        match k {
                            "fill" => node_style.fill = Some(v.to_string()),
                            "stroke" => node_style.stroke = Some(v.to_string()),
                            "color" => node_style.color = Some(v.to_string()),
                            _ => {}
                        }
                    }
                }
                styles.insert(node_id.to_string(), node_style);
            }
            continue;
        }

        let parsed_chains = parse_flowchart_line_chains(trimmed);
        if !parsed_chains.is_empty() {
            for (from_raw, to_raw, e_type, e_label) in parsed_chains {
                let (from_id, from_label, from_shape) = extract_node(&from_raw);
                let (to_id, to_label, to_shape) = extract_node(&to_raw);

                nodes.entry(from_id.clone()).or_insert_with(|| Node {
                    id: from_id.clone(),
                    label: from_label.unwrap_or_else(|| from_id.clone()),
                    shape: from_shape,
                    style: None,
                });

                nodes.entry(to_id.clone()).or_insert_with(|| Node {
                    id: to_id.clone(),
                    label: to_label.unwrap_or_else(|| to_id.clone()),
                    shape: to_shape,
                    style: None,
                });

                edges.push(Edge {
                    from: from_id,
                    to: to_id,
                    label: e_label,
                    edge_type: e_type,
                });
            }
        } else {
            let (id, label, shape) = extract_node(trimmed);
            if !id.is_empty() {
                nodes.entry(id.clone()).or_insert_with(|| Node {
                    id: id.clone(),
                    label: label.unwrap_or_else(|| id.clone()),
                    shape,
                    style: None,
                });
            }
        }
    }

    for (id, style) in styles {
        if let Some(node) = nodes.get_mut(&id) {
            node.style = Some(style);
        }
    }

    Ok(ParsedDiagram::Flowchart(Flowchart {
        direction,
        nodes: nodes.into_values().collect(),
        edges,
    }))
}

fn parse_flowchart_line_chains(line: &str) -> Vec<(String, String, EdgeType, Option<String>)> {
    let mut results = Vec::new();
    let mut remaining = line.trim();

    let edge_delims: &[(&str, EdgeType)] = &[
        ("-.->", EdgeType::DottedArrow),
        ("==>", EdgeType::ThickArrow),
        ("-->", EdgeType::SolidArrow),
        ("---", EdgeType::SolidLine),
    ];

    loop {
        let mut earliest: Option<(usize, &str, EdgeType)> = None;
        for &(delim, edge_type) in edge_delims {
            if let Some(pos) = remaining.find(delim) {
                match earliest {
                    None => earliest = Some((pos, delim, edge_type)),
                    Some((min_pos, _, _)) if pos < min_pos => earliest = Some((pos, delim, edge_type)),
                    _ => {}
                }
            }
        }

        if let Some((pos, delim, edge_type)) = earliest {
            let from_node = remaining[..pos].trim().to_string();
            let after_delim = remaining[pos + delim.len()..].trim();

            let (edge_label, after_label) = if after_delim.starts_with('|') {
                if let Some(end_bar) = after_delim[1..].find('|') {
                    let lbl = after_delim[1..=end_bar].trim().to_string();
                    let rest = after_delim[end_bar + 2..].trim();
                    (Some(lbl), rest)
                } else {
                    (None, after_delim)
                }
            } else {
                (None, after_delim)
            };

            let mut next_edge_pos: Option<usize> = None;
            for &(next_delim, _) in edge_delims {
                if let Some(p) = after_label.find(next_delim) {
                    match next_edge_pos {
                        None => next_edge_pos = Some(p),
                        Some(min_p) if p < min_p => next_edge_pos = Some(p),
                        _ => {}
                    }
                }
            }

            let (to_node, next_remaining) = match next_edge_pos {
                Some(p) => (after_label[..p].trim().to_string(), &after_label[p..]),
                None => (after_label.to_string(), ""),
            };

            if !from_node.is_empty() && !to_node.is_empty() {
                results.push((from_node.clone(), to_node.clone(), edge_type, edge_label));
            }

            if next_remaining.is_empty() {
                break;
            } else {
                remaining = after_label;
            }
        } else {
            break;
        }
    }

    results
}

fn extract_node(s: &str) -> (String, Option<String>, NodeShape) {
    let s = s.trim();

    if let (Some(start), Some(end)) = (s.find("[("), s.rfind(")]")) {
        let id = s[..start].trim().to_string();
        let label = s[start + 2..end].trim().to_string();
        return (id, Some(label), NodeShape::Cylinder);
    }
    if let (Some(start), Some(end)) = (s.find("(("), s.rfind("))")) {
        let id = s[..start].trim().to_string();
        let label = s[start + 2..end].trim().to_string();
        return (id, Some(label), NodeShape::Circle);
    }
    if let (Some(start), Some(end)) = (s.find('{'), s.rfind('}')) {
        let id = s[..start].trim().to_string();
        let label = s[start + 1..end].trim().to_string();
        return (id, Some(label), NodeShape::Diamond);
    }
    if let (Some(start), Some(end)) = (s.find('('), s.rfind(')')) {
        let id = s[..start].trim().to_string();
        let label = s[start + 1..end].trim().to_string();
        return (id, Some(label), NodeShape::Rounded);
    }
    if let (Some(start), Some(end)) = (s.find('['), s.rfind(']')) {
        let id = s[..start].trim().to_string();
        let label = s[start + 1..end].trim().to_string();
        return (id, Some(label), NodeShape::Rectangle);
    }

    (s.to_string(), None, NodeShape::Rectangle)
}

fn parse_sequence(lines: &[&str]) -> Result<ParsedDiagram, String> {
    let mut participants = Vec::new();
    let mut items = Vec::new();

    for line in &lines[1..] {
        let trimmed = line.trim();
        if trimmed.starts_with("participant ") {
            let name = trimmed.trim_start_matches("participant").trim();
            if !participants.contains(&name.to_string()) {
                participants.push(name.to_string());
            }
        } else if trimmed.starts_with("Note ") {
            let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
            let text = if parts.len() > 1 { parts[1].trim().to_string() } else { "".to_string() };
            let prefix = parts[0].trim_start_matches("Note").trim();
            let mut over = Vec::new();
            if prefix.starts_with("over ") {
                let targets = prefix.trim_start_matches("over ").trim();
                for t in targets.split(',') {
                    let target_name = t.trim().to_string();
                    if !participants.contains(&target_name) {
                        participants.push(target_name.clone());
                    }
                    over.push(target_name);
                }
            } else if prefix.starts_with("right of ") {
                let target_name = prefix.trim_start_matches("right of ").trim().to_string();
                if !participants.contains(&target_name) {
                    participants.push(target_name.clone());
                }
                over.push(target_name);
            }
            items.push(SequenceItem::Note { over, text });
        } else if trimmed.starts_with("loop") {
            let label = trimmed.trim_start_matches("loop").trim().to_string();
            items.push(SequenceItem::LoopStart { label });
        } else if trimmed == "end" {
            items.push(SequenceItem::LoopEnd);
        } else if trimmed.starts_with("alt") {
            let label = trimmed.trim_start_matches("alt").trim().to_string();
            items.push(SequenceItem::AltStart { label });
        } else if trimmed.starts_with("else") {
            let label = trimmed.trim_start_matches("else").trim().to_string();
            items.push(SequenceItem::AltElse { label });
        } else if trimmed.contains("->>") || trimmed.contains("->") || trimmed.contains("-->>") || trimmed.contains("-->") {
            let arrow = if trimmed.contains("-->>") {
                "-->>"
            } else if trimmed.contains("->>") {
                "->>"
            } else if trimmed.contains("-->") {
                "-->"
            } else {
                "->"
            };

            let parts: Vec<&str> = trimmed.split(arrow).collect();
            if parts.len() == 2 {
                let from = parts[0].trim().to_string();
                let right = parts[1].trim();

                let (to, label) = if let Some(colon) = right.find(':') {
                    (right[..colon].trim().to_string(), right[colon + 1..].trim().to_string())
                } else {
                    (right.to_string(), "".to_string())
                };

                if !participants.contains(&from) {
                    participants.push(from.clone());
                }
                if !participants.contains(&to) {
                    participants.push(to.clone());
                }

                items.push(SequenceItem::Message(SequenceMessage {
                    from,
                    to,
                    label,
                    arrow: arrow.to_string(),
                }));
            }
        }
    }

    Ok(ParsedDiagram::Sequence(SequenceDiagram {
        participants,
        items,
    }))
}

fn parse_class_diagram(lines: &[&str]) -> Result<ParsedDiagram, String> {
    let mut classes = std::collections::HashMap::new();
    let mut relations = Vec::new();
    let mut current_class: Option<String> = None;

    for line in &lines[1..] {
        let trimmed = line.trim();
        if trimmed.starts_with("class ") {
            if trimmed.contains('{') {
                let name = trimmed.trim_start_matches("class ").trim_end_matches('{').trim().to_string();
                classes.entry(name.clone()).or_insert_with(|| ClassItem { name: name.clone(), members: Vec::new() });
                current_class = Some(name);
            } else {
                let name = trimmed.trim_start_matches("class ").trim().to_string();
                classes.entry(name.clone()).or_insert_with(|| ClassItem { name, members: Vec::new() });
            }
        } else if trimmed == "}" {
            current_class = None;
        } else if let Some(ref class_name) = current_class {
            let is_method = trimmed.contains('(');
            let vis = match trimmed.chars().next() {
                Some(c) if c == '+' || c == '-' || c == '#' || c == '~' => c,
                _ => '+',
            };
            let member_str = trimmed.trim_start_matches(['+', '-', '#', '~']).trim();
            let (m_type, name) = if let Some(space) = member_str.find(' ') {
                (member_str[..space].to_string(), member_str[space + 1..].to_string())
            } else {
                ("".to_string(), member_str.to_string())
            };

            if let Some(c) = classes.get_mut(class_name) {
                c.members.push(ClassMember {
                    visibility: vis,
                    name,
                    member_type: m_type,
                    is_method,
                });
            }
        } else if trimmed.contains("<|--") || trimmed.contains("*--") || trimmed.contains("o--") || trimmed.contains("-->") {
            let rel_symbol = if trimmed.contains("<|--") {
                "<|--"
            } else if trimmed.contains("*--") {
                "*--"
            } else if trimmed.contains("o--") {
                "o--"
            } else {
                "-->"
            };

            let parts: Vec<&str> = trimmed.split(rel_symbol).collect();
            if parts.len() == 2 {
                let from = parts[0].trim().to_string();
                let right = parts[1].trim();
                let (to, label) = if let Some(colon) = right.find(':') {
                    (right[..colon].trim().to_string(), Some(right[colon + 1..].trim().to_string()))
                } else {
                    (right.to_string(), None)
                };

                classes.entry(from.clone()).or_insert_with(|| ClassItem { name: from.clone(), members: Vec::new() });
                classes.entry(to.clone()).or_insert_with(|| ClassItem { name: to.clone(), members: Vec::new() });

                relations.push(ClassRelation {
                    from,
                    to,
                    relation_type: rel_symbol.to_string(),
                    label,
                });
            }
        }
    }

    Ok(ParsedDiagram::Class(ClassDiagram {
        classes: classes.into_values().collect(),
        relations,
    }))
}
