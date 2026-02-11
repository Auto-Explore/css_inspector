# css/css-conditional/container-queries/query-evaluation-style.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/query-evaluation-style.html"
}
```

## style[0]

```css

  @property --length {
    syntax: "<length>";
    inherits: false;
    initial-value: 3px;
  }
  @property --length-inherited {
    syntax: "<length>";
    inherits: true;
    initial-value: 3px;
  }
  @property --percentage {
    syntax: "<percentage>";
    inherits: true;
    initial-value: 30%;
  }
  @property --number {
    syntax: "<number>";
    inherits: true;
    initial-value: 3;
  }
  @property --angle {
    syntax: "<angle>";
    inherits: true;
    initial-value: 3deg;
  }
  @property --time {
    syntax: "<time>";
    inherits: true;
    initial-value: 3s;
  }
  @property --resolution {
    syntax: "<resolution>";
    inherits: true;
    initial-value: 3dpi;
  }
  #container {
    --applied: false;
    --foo: bar;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
