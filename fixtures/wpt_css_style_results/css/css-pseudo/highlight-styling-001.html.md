# css/css-pseudo/highlight-styling-001.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-styling-001.html"
}
```

## style[0]

```css

    main {
        --x: red;
        font-size: 7em;
        margin: 0.5em;
    }
    main::selection {
        --x: green;
        color: white;
        background-color: var(--x, blue);
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
