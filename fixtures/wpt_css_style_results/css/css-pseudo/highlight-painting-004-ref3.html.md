# css/css-pseudo/highlight-painting-004-ref3.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-painting-004-ref3.html"
}
```

## style[0]

```css

    * {
        text-decoration-skip-ink: none;
        text-decoration-skip: none;
    }
    main {
        font-size: 7em;
        margin: 0.5em;
        width: min-content;
    }
    .unselected {
        text-decoration: #E03838C0 wavy underline;
        color: #C0C000C0;
    }
    .selected {
        background: #38E038C0;
        text-decoration: #3838E0C0 wavy underline;
    }
    .selected, .selected * {
        color: #3838E0C0;
    }
    .selection {
        text-decoration: #663399C0 wavy line-through;
    }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
