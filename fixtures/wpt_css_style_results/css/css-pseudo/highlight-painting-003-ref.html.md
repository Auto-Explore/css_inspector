# css/css-pseudo/highlight-painting-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-painting-003-ref.html"
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
    .background {
        width: 100%;
        height: 0.25em;
        background: #707070C0;
    }
    .unselected {
        color: #E03838C0;
        text-decoration: #C0C000C0 solid line-through;
    }
    .selected {
        background: #38E038C0;
        text-decoration: #663399C0 solid line-through;
    }
    .selected, .selected * {
        color: #663399C0;
    }
    .selection {
        text-decoration: #3838E0C0 solid underline;
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
