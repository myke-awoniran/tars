# tars::A Django-Style Template Engine

`tars` is a lightweight, Django-style template engine designed to transform templates and structured data into HTML
output.  
It supports familiar templating constructs—variables, control flow, loops, includes, and filters.

---

## Table of Contents

- [Overview](#overview)
- [Core Concepts](#core-concepts)
- [Template Syntax](#template-syntax)
- [Engine Architecture](#engine-architecture)
- [Rendering Flow](#rendering-flow)
- [Compile-Time vs Runtime Modes](#compile-time-vs-runtime-modes)
- [Design Goals](#design-goals)
- [Non-Goals](#non-goals)
- [Future Extensions](#future-extensions)

---

## Overview

At its core, `tars` takes:

- **A template** written in a Django-style syntax
- **A datasource (context)** containing structured data

…and produces:

- **HTML output**

`tars` treats templates as a **domain-specific programming language whose only job is to produce text**.

---

## Core Concepts

- Templates consist of **static literals** and **dynamic constructs**
- Templates are **parsed into an Abstract Syntax Tree (AST)**
- Rendering is performed by **evaluating the AST against a context**
- The same compiled template can be reused with different data

---

## Template Syntax

`tars` follows a familiar Django-style syntax.

### 1. Static Literals

Anything not marked as a template construct is treated as literal text:

```html
<h1>Michael Awoniran</h1>
```

## 2. Template Variables

Template variables are enclosed in **double curly braces**:

```html
<p>{{ name }}</p>
```

- Variables are resolved from the provided context
- If a variable does not exist, behavior is engine-defined (error or empty output)
- Nested Access
    - Nested attribute access is supported using dot notation:
    ```<p>{{ customer.name }}</p> 
    ```

## 3. Conditional Logic

Conditional blocks use {% if %} and {% endif %}:

```{% if amount > 1000 %}
    <p>High value transaction</p>
    {% endif %}
```

## Condition Support

- Conditions may include:
- Variables
- Literals
- Comparison operators:
  > < == != >

## 5. Template Includes

Templates can include other templates using the include tag:
{% include "footer.html" %}

## 6. Filters

Filters transform variable output:
{{ name | upper }}

## Engine Architecture

The `tars` template engine is composed of **two core components**:

1. **Parser**
2. **HTML Generator**

These components work together to transform a template and structured data into a final HTML output.

---

### Components and Responsibilities

#### 1. Parser

- Takes the **template string** as input
- Parses the template syntax (literals, variables, control structures)
- Produces a **parsed representation** (tokens / parsed structures) suitable for rendering
- Does **not** perform rendering or data injection

#### 2. HTML Generator

- Takes:
    - The **parsed output** from the parser
    - The **structured data (context)**
- Combines static literals with dynamic values from the context
- Evaluates control logic (conditions, loops, includes, filters)
- Produces the **final output string**
- Writes the generated string to an **HTML file**

---

### Data Flow Overview

1. The template file is read as a **template string**
2. The template string is passed to the **parser**
3. The parser processes the template and outputs a parsed structure
4. The parsed structure is passed to the **HTML generator**
5. Structured data is provided to the generator as **context**
6. The generator produces the final HTML string
7. The output string is written to the HTML file

---

### Architecture Diagram

The diagram below illustrates the flow of data through the engine:

## Engine Architecture

```mermaid
flowchart TD
    T[Template File]
    P[Parser]
    G[HTML Generator]
    O[HTML Output File]
    C[Structured Data]

    T --> P
    P -->|Parsed Template| G
    C --> G
    G --> O


