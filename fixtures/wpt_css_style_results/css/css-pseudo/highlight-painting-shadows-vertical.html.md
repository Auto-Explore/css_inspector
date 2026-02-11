# css/css-pseudo/highlight-painting-shadows-vertical.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-painting-shadows-vertical.html"
}
```

## style[0]

```css

    :root {
        font-family: Ahem;
        writing-mode: vertical-lr;
        line-height: 1;
        white-space: pre;
    }
    p {
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
