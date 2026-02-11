# css/css-overflow/scroll-markers/root-scroll-marker.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/root-scroll-marker.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  :root {
    scroll-marker-group: before;
  }

  :root::scroll-marker-group {
    border: 3px solid black;
    padding: 5px;
    height: 20px;
    display: block;
    position: fixed;
    top: 0;
  }

  div {
    width: 600px;
    height: 300px;
    margin-bottom: 20px;
    background: green;
  }

  div:first-of-type {
    margin-top: 40px;
  }

  div::scroll-marker {
    content: "";
    width: 10px;
    height: 10px;
    background-color: blue;
    border-radius: 100%;
    display: inline-block;
    margin-right: 4px;
  }

  div:last-of-type::scroll-marker {
    margin-right: 0px;
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
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
