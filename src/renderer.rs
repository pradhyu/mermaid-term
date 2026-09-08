use crate::color::Ansi;
use crate::parser::*;

pub fn render_ascii(diagram: &ParsedDiagram) -> String {
    match diagram {
        ParsedDiagram::Flowchart(fc) => render_flowchart_ascii(fc),
        ParsedDiagram::Sequence(seq) => render_sequence_ascii(seq),
        ParsedDiagram::Class(cls) => render_class_ascii(cls),
    }
}

fn format_node_box(node: &Node) -> Vec<String> {
    let lbl = &node.label;
    let len = lbl.len();

    let (border_color, text_color, reset) = if let Some(ref style) = node.style {
        let b_color = style
            .stroke
            .as_deref()
            .or(style.fill.as_deref())
            .map(Ansi::parse_color)
            .unwrap_or_else(|| Ansi::CYAN.to_string());

        let t_color = style
            .color
            .as_deref()
            .map(Ansi::parse_color)
            .unwrap_or_else(|| Ansi::WHITE.to_string());

        (b_color, t_color, Ansi::RESET)
    } else {
        (Ansi::CYAN.to_string(), Ansi::BOLD.to_string(), Ansi::RESET)
    };

    match node.shape {
        NodeShape::Diamond => {
            vec![
                format!("{}  ╱{:─<w$}╲{}", border_color, "", reset, w = len + 2),
                format!("{} ⟨  {}{}{}  ⟩{}", border_color, text_color, lbl, border_color, reset),
                format!("{}  ╲{:─<w$}╱{}", border_color, "", reset, w = len + 2),
            ]
        }
        NodeShape::Rounded => {
            vec![
                format!("{} ╭─{:─<w$}─╮{}", border_color, "", reset, w = len),
                format!("{} │ {}{}{} │{}", border_color, text_color, lbl, border_color, reset),
                format!("{} ╰─{:─<w$}─╯{}", border_color, "", reset, w = len),
            ]
        }
        NodeShape::Cylinder => {
            vec![
                format!("{} ⌠─{:─<w$}─⌡{}", border_color, "", reset, w = len),
                format!("{} │ {}{}{} │{}", border_color, text_color, lbl, border_color, reset),
                format!("{} ⌡─{:─<w$}─⌠{}", border_color, "", reset, w = len),
            ]
        }
        NodeShape::Circle => {
            vec![
                format!("{} (( {:─<w$} )){}", border_color, "", reset, w = len),
                format!("{} (( {}{}{} )){}", border_color, text_color, lbl, border_color, reset),
                format!("{} (( {:─<w$} )){}", border_color, "", reset, w = len),
            ]
        }
        NodeShape::Rectangle => {
            vec![
                format!("{}┌─{:─<w$}─┐{}", border_color, "", reset, w = len),
                format!("{}│ {}{}{} │{}", border_color, text_color, lbl, border_color, reset),
                format!("{}└─{:─<w$}─┘{}", border_color, "", reset, w = len),
            ]
        }
    }
}

fn render_flowchart_ascii(fc: &Flowchart) -> String {
    let mut out = String::new();

    let get_node = |id: &str| -> Node {
        fc.nodes
            .iter()
            .find(|n| n.id == id)
            .cloned()
            .unwrap_or_else(|| Node {
                id: id.to_string(),
                label: id.to_string(),
                shape: NodeShape::Rectangle,
                style: None,
            })
    };

    match fc.direction {
        Direction::LeftToRight => {
            for (i, edge) in fc.edges.iter().enumerate() {
                let from_node = get_node(&edge.from);
                let to_node = get_node(&edge.to);

                let from_box = format_node_box(&from_node);
                let to_box = format_node_box(&to_node);

                let arrow_str = match (&edge.edge_type, &edge.label) {
                    (EdgeType::DottedArrow, Some(lbl)) => {
                        format!("{} -.- {}{}{} .-> {}", Ansi::YELLOW, Ansi::BRIGHT_YELLOW, lbl, Ansi::YELLOW, Ansi::RESET)
                    }
                    (EdgeType::DottedArrow, None) => format!("{} -.-.-.-> {}", Ansi::YELLOW, Ansi::RESET),
                    (EdgeType::ThickArrow, Some(lbl)) => {
                        format!("{} === {}{}{} ==> {}", Ansi::BRIGHT_GREEN, Ansi::WHITE, lbl, Ansi::BRIGHT_GREEN, Ansi::RESET)
                    }
                    (EdgeType::ThickArrow, None) => format!("{} ========> {}", Ansi::BRIGHT_GREEN, Ansi::RESET),
                    (EdgeType::SolidArrow, Some(lbl)) => {
                        format!("{} ─── {}{}{} ───▶ {}", Ansi::BLUE, Ansi::WHITE, lbl, Ansi::BLUE, Ansi::RESET)
                    }
                    (EdgeType::SolidArrow, None) => format!("{} ────────▶ {}", Ansi::BLUE, Ansi::RESET),
                    (EdgeType::SolidLine, Some(lbl)) => {
                        format!("{} ─── {}{}{} ─── {}", Ansi::DIM, Ansi::WHITE, lbl, Ansi::DIM, Ansi::RESET)
                    }
                    (EdgeType::SolidLine, None) => format!("{} ───────── {}", Ansi::DIM, Ansi::RESET),
                };

                for row in 0..3 {
                    let left_row = &from_box[row];
                    let right_row = &to_box[row];
                    let mid = if row == 1 {
                        arrow_str.as_str()
                    } else {
                        "           "
                    };
                    out.push_str(&format!("{}{}{}\n", left_row, mid, right_row));
                }

                if i < fc.edges.len() - 1 {
                    out.push('\n');
                }
            }
        }
        _ => {
            for (i, edge) in fc.edges.iter().enumerate() {
                let from_node = get_node(&edge.from);
                let to_node = get_node(&edge.to);

                let from_box = format_node_box(&from_node);
                let to_box = format_node_box(&to_node);

                for row in &from_box {
                    out.push_str(row);
                    out.push('\n');
                }

                if let Some(lbl) = &edge.label {
                    out.push_str(&format!("{}     │  {}{}{}\n", Ansi::DIM, Ansi::YELLOW, lbl, Ansi::RESET));
                }

                match edge.edge_type {
                    EdgeType::DottedArrow => out.push_str(&format!("{}     :\n     ▼{}\n", Ansi::YELLOW, Ansi::RESET)),
                    EdgeType::ThickArrow => out.push_str(&format!("{}     ║\n     ▼{}\n", Ansi::BRIGHT_GREEN, Ansi::RESET)),
                    EdgeType::SolidArrow => out.push_str(&format!("{}     │\n     ▼{}\n", Ansi::BLUE, Ansi::RESET)),
                    EdgeType::SolidLine => out.push_str(&format!("{}     │\n     │{}\n", Ansi::DIM, Ansi::RESET)),
                }

                for row in &to_box {
                    out.push_str(row);
                    out.push('\n');
                }

                if i < fc.edges.len() - 1 {
                    out.push('\n');
                }
            }
        }
    }

    if fc.edges.is_empty() {
        for node in &fc.nodes {
            for row in format_node_box(node) {
                out.push_str(&row);
                out.push('\n');
            }
            out.push('\n');
        }
    }

    out
}

fn render_sequence_ascii(seq: &SequenceDiagram) -> String {
    if seq.participants.is_empty() {
        return "Empty sequence diagram".to_string();
    }

    let mut out = String::new();
    let col_width = 18;
    let mut header_boxes = String::new();
    let mut header_lines = String::new();
    let mut header_bots = String::new();

    for p in &seq.participants {
        let label = if p.len() > col_width - 4 {
            &p[..col_width - 4]
        } else {
            p
        };
        let pad = (col_width - 2).saturating_sub(label.len() + 2);
        let pad_left = pad / 2;
        let pad_right = pad - pad_left;

        header_boxes.push_str(&format!("{}┌{:─<width$}┐{} ", Ansi::CYAN, "", Ansi::RESET, width = col_width - 2));
        header_lines.push_str(&format!("{}│{}{:pad_l$}{}{}{:pad_r$}{}│{} ", Ansi::CYAN, Ansi::RESET, "", Ansi::BOLD, label, "", Ansi::CYAN, Ansi::RESET, pad_l = pad_left, pad_r = pad_right));
        header_bots.push_str(&format!("{}└{:─<width$}┘{} ", Ansi::CYAN, "", Ansi::RESET, width = col_width - 2));
    }

    out.push_str(&header_boxes);
    out.push('\n');
    out.push_str(&header_lines);
    out.push('\n');
    out.push_str(&header_bots);
    out.push('\n');

    let col_center = |idx: usize| -> usize {
        idx * (col_width + 1) + col_width / 2
    };

    let total_width = seq.participants.len() * (col_width + 1);

    let mut line_row = vec![' '; total_width];
    for i in 0..seq.participants.len() {
        let c = col_center(i);
        if c < line_row.len() {
            line_row[c] = '│';
        }
    }
    let lifeline_str: String = line_row.iter().collect();
    out.push_str(&format!("{}{}{}", Ansi::DIM, lifeline_str, Ansi::RESET));
    out.push('\n');

    for item in &seq.items {
        match item {
            SequenceItem::Message(msg) => {
                let from_idx = seq.participants.iter().position(|p| p == &msg.from);
                let to_idx = seq.participants.iter().position(|p| p == &msg.to);

                if let (Some(f), Some(t)) = (from_idx, to_idx) {
                    let start = col_center(f.min(t));
                    let end = col_center(f.max(t));

                    let mut label_row = line_row.clone();
                    let label_start = start + 2;
                    for (offset, ch) in msg.label.chars().enumerate() {
                        if label_start + offset < end {
                            label_row[label_start + offset] = ch;
                        }
                    }
                    let s: String = label_row.iter().collect();
                    out.push_str(&format!("{}{}{}", Ansi::BRIGHT_YELLOW, s, Ansi::RESET));
                    out.push('\n');

                    let mut arrow_row = line_row.clone();
                    if f < t {
                        for x in (start + 1)..end {
                            arrow_row[x] = '─';
                        }
                        arrow_row[end] = '▶';
                    } else {
                        for x in (start + 1)..end {
                            arrow_row[x] = '─';
                        }
                        arrow_row[start] = '◀';
                    }
                    let s: String = arrow_row.iter().collect();
                    out.push_str(&format!("{}{}{}", Ansi::GREEN, s, Ansi::RESET));
                    out.push('\n');
                    out.push_str(&format!("{}{}{}", Ansi::DIM, lifeline_str, Ansi::RESET));
                    out.push('\n');
                }
            }
            SequenceItem::LoopStart { label } => {
                out.push_str(&format!("{} ╭── loop [{}{}{}] {:─<w$}╮{}\n", Ansi::MAGENTA, Ansi::WHITE, label, Ansi::MAGENTA, "", Ansi::RESET, w = total_width.saturating_sub(label.len() + 16)));
            }
            SequenceItem::LoopEnd => {
                out.push_str(&format!("{} ╰{:─<w$}╯{}\n", Ansi::MAGENTA, "", Ansi::RESET, w = total_width.saturating_sub(4)));
            }
            SequenceItem::AltStart { label } => {
                out.push_str(&format!("{} ╭── alt [{}{}{}] {:─<w$}╮{}\n", Ansi::YELLOW, Ansi::WHITE, label, Ansi::YELLOW, "", Ansi::RESET, w = total_width.saturating_sub(label.len() + 15)));
            }
            SequenceItem::AltElse { label } => {
                out.push_str(&format!("{} ├── else [{}{}{}] {:─<w$}┤{}\n", Ansi::YELLOW, Ansi::WHITE, label, Ansi::YELLOW, "", Ansi::RESET, w = total_width.saturating_sub(label.len() + 16)));
            }
            SequenceItem::AltEnd => {
                out.push_str(&format!("{} ╰{:─<w$}╯{}\n", Ansi::YELLOW, "", Ansi::RESET, w = total_width.saturating_sub(4)));
            }
            SequenceItem::Note { text, .. } => {
                out.push_str(&format!("{} [Note: {}]{}\n", Ansi::BRIGHT_CYAN, text, Ansi::RESET));
            }
        }
    }

    out
}

fn render_class_ascii(cls: &ClassDiagram) -> String {
    let mut out = String::new();

    for c in &cls.classes {
        let max_w = c.members.iter().map(|m| m.name.len() + m.member_type.len() + 4).max().unwrap_or(0).max(c.name.len() + 4);

        out.push_str(&format!("{}┌{:─<w$}┐{}\n", Ansi::CYAN, "", Ansi::RESET, w = max_w));
        out.push_str(&format!("{}│ {}{:^w$}{} │{}\n", Ansi::CYAN, Ansi::BOLD, c.name, Ansi::CYAN, Ansi::RESET, w = max_w - 2));
        out.push_str(&format!("{}├{:─<w$}┤{}\n", Ansi::CYAN, "", Ansi::RESET, w = max_w));

        for m in &c.members {
            let m_str = if m.is_method {
                format!("{}{} {}{}{}", Ansi::GREEN, m.visibility, Ansi::WHITE, m.name, if m.member_type.is_empty() { "".to_string() } else { format!(": {}{}", Ansi::DIM, m.member_type) })
            } else {
                format!("{}{} {}{}{}", Ansi::YELLOW, m.visibility, Ansi::WHITE, m.name, if m.member_type.is_empty() { "".to_string() } else { format!(": {}{}", Ansi::DIM, m.member_type) })
            };
            out.push_str(&format!("{}│ {}{:<w$}{} │{}\n", Ansi::CYAN, m_str, "", Ansi::CYAN, Ansi::RESET, w = max_w.saturating_sub(m.name.len() + m.member_type.len() + 6)));
        }

        out.push_str(&format!("{}└{:─<w$}┘{}\n\n", Ansi::CYAN, "", Ansi::RESET, w = max_w));
    }

    for rel in &cls.relations {
        let lbl_str = if let Some(ref l) = rel.label { format!(" : {}", l) } else { "".to_string() };
        out.push_str(&format!("{}{} {}{}{} {}{}{}\n", Ansi::BOLD, rel.from, Ansi::BRIGHT_GREEN, rel.relation_type, Ansi::BOLD, rel.to, Ansi::DIM, lbl_str));
    }

    out
}
