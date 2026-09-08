# Microservices Architecture & Flow

This document details the system design for our event-driven architecture.

## 1. Authentication Handshake
```mermaid
sequenceDiagram
    participant App
    participant Gateway
    participant IdentityProvider
    App->>Gateway: Bearer Token Request
    Gateway->>IdentityProvider: Validate OAuth2 Signature
    IdentityProvider-->>Gateway: Claims & Scopes Validated
    Gateway-->>App: Access Granted (Session Ready)
```

## 2. Order Fulfillment Pipeline
```mermaid
graph TD
    OrderPlaced[New Order Event] --> PaymentGate[Process Stripe Payment]
    PaymentGate --> Inventory[Reserve Warehouse Stock]
    Inventory --> Shipping[Generate Tracking Label]
    Shipping --> Notification[Send Customer Email]
```

## 3. High Availability Deployment
```mermaid
graph LR
    DNS[Route 53] --> Edge[Cloudflare CDN]
    Edge --> PrimaryRegion[US-East Cluster]
    Edge --> FailoverRegion[US-West Cluster]
```
