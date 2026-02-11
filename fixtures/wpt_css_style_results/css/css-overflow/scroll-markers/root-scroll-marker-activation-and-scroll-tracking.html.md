# css/css-overflow/scroll-markers/root-scroll-marker-activation-and-scroll-tracking.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/root-scroll-marker-activation-and-scroll-tracking.html"
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
    background: blue;
  }

  div:first-of-type {
    margin-top: 40px;
  }

  div::scroll-marker {
    content: "";
    width: 20px;
    height: 20px;
    background-color: red;
    display: inline-block;
    margin-right: 4px;
  }

  div::scroll-marker:target-current {
    background-color: green;
  }

  div:last-of-type::scroll-marker {
    margin-right: 0px;
  }
```

```json
{
  "errors": 5,
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
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
