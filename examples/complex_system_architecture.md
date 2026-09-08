# Global Production Infrastructure & Event Flow

This technical specification details our worldwide multi-region cluster and event-driven microservice mesh.

---

## 1. Global Ingress & Traffic Management
```mermaid
graph LR
    User[Global Web Clients] --> Anycast[Cloudflare Anycast DNS]
    Anycast --> WAF{Cloudflare WAF Check}
    WAF -.->|Malicious IP| Block[Drop Connection / 403]
    WAF ==>|Legitimate| OriginLB[AWS NLB Ingress]
    OriginLB --> K8sGateway[Envoy Ingress Gateway]
    
    style User stroke:cyan,color:white
    style Block stroke:red,color:red
    style K8sGateway stroke:#00ff88,color:white
```

---

## 2. Distributed Consensus & Order Processing
```mermaid
sequenceDiagram
    participant Web as Web Frontend
    participant API as API Gateway
    participant OrderSvc as Order Service
    participant PaySvc as Payment Engine
    participant EventBus as Apache Kafka

    Web->>API: POST /checkout (Cart Payload)
    API->>OrderSvc: Create Pending Order
    OrderSvc->>PaySvc: Authorize Card Hold ($150)
    Note over PaySvc: Contact Stripe API with idempotency key
    PaySvc-->>OrderSvc: 200 Card Authorized
    
    loop Distributed Outbox Delivery
        OrderSvc->>EventBus: Publish `order.created` Event
        EventBus-->>OrderSvc: Kafka ACK (Leader Written)
    end
    
    alt Payment Cleared
        OrderSvc-->>API: Order #9821 Confirmed
        API-->>Web: 200 OK (Tracking URL)
    else Payment Declined
        OrderSvc-->>API: 402 Payment Failed
        API-->>Web: Show Retry Checkout Modal
    end
```

---

## 3. Storage Layer Domain Hierarchy
```mermaid
classDiagram
    class DatabaseEntity {
        +UUID id
        +DateTime createdAt
        +DateTime updatedAt
        +save() bool
        +delete() bool
    }
    
    class Customer {
        +String firstName
        +String lastName
        +String email
        +authenticate() bool
    }
    
    class Subscription {
        +PlanTier tier
        +DateTime currentPeriodEnd
        +bool cancelAtPeriodEnd
        +upgradeTier(PlanTier next) bool
    }
    
    DatabaseEntity <|-- Customer : inherits
    DatabaseEntity <|-- Subscription : inherits
    Customer *-- Subscription : has
```
