# css/css-pseudo/target-text-shadow-vertical.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/target-text-shadow-vertical.html"
}
```

## style[0]

```css

    :root {
        line-height: 1;
        writing-mode: vertical-lr;
    }
    p {
        font-size: 2em;
        color: black;
        text-shadow: 0.1em 0.1em 3px rgba(0,0,0,0.5);
    }
    p::target-text {
        color: green;
        text-shadow: 0.25em 0.25em 3px rgba(0,0,128,0.5);
    }
```

```json
{
  "errors": 2,
  "messages": [
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
