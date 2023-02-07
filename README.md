```mermaid
---
title: Tokens
---
flowchart LR

    1 --> + --> 3 --> - --> 4
```

```mermaid
---
title: AST
---
flowchart TB
    + --> 1
    + --> -
    - --> 3
    - --> 4
```
