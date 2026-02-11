# css/css-pseudo/highlight-painting-shadows-horizontal.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-painting-shadows-horizontal.html"
}
```

## style[0]

```css

    :root {
        line-height: 1;
        white-space: pre;
    }
    p {
        font-size: 2em;
        color: black;
        text-shadow: 0.1em 0.1em rgba(0,0,0,0.5);
        position: absolute;
        top: 10px;
        left: 10px;
    }
    p::selection {
        color: green;
        text-shadow: -0.25em -0.25em rgba(0,128,0,0.5);
    }
    p::target-text {
        color: blue;
        text-shadow: 0.25em 0.25em rgba(0,0,128,0.5);
    }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
