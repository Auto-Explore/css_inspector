# css/CSS2/syntax/declarations-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/declarations-009.xht"
}
```

## style[0]

```css

    #a {
      color: green;
      @import "support/import-red.css" color: red;
    }
    #b {
      color: red;
      @import "support/import-red.css";
      color: green;
    }
    #c {
      color: green;
      @media { #c { color: red !important } }
      color: red;
    }
    #d {
      color: red;
      @media { #c { color: red !important } };
      color: green;
    }
    #e {
      color: green;
      @foo [ color: red; } #e { color: red; } ]
    }
    #f {
      color: green;
      color: red @import "support/import-red.css";
    }
  
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid property name in declaration.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    },
    {
      "message": "Invalid property name in declaration.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
