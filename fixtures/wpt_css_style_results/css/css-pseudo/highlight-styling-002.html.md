# css/css-pseudo/highlight-styling-002.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-styling-002.html"
}
```

## style[0]

```css

    main {
        font-size: 7em;
        margin: 0.5em;
        --x: green;
    }
    main::selection {
        color: white;
        background-color: var(--x, red);
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
