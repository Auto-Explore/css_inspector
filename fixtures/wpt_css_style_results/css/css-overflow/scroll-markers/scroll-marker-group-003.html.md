# css/css-overflow/scroll-markers/scroll-marker-group-003.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-003.html"
}
```

## style[0]

```css

  :root {
    --after-background: red;
    --scroll-marker-group-background: red;
  }

  div {
    scroll-marker-group: after;
  }

  div>* {
    background: green;
    display: flex;
    height: 20px;
    width: 100px;
  }

  div.before::before {
    background: green;
    content: "";
    display: flex;
    height: 20px;
    width: 100px;
  }

  div.after::after {
    background: var(--after-background);
    content: "";
    display: flex;
    height: 20px;
    width: 100px;
  }

  div.scroll-marker-group::scroll-marker-group {
    background: var(--scroll-marker-group-background);
    display: flex;
    height: 20px;
    width: 100px;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “scroll-marker-group”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
