# css/css-overflow/scroll-markers/scroll-target-group-003.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-target-group-003.html"
}
```

## style[0]

```css

  :root {
    font-family: "Ahem";
  }

  #wrapper {
    scroll-target-group: auto;
  }

  #scroller {
    overflow: auto;
    height: 130px;
    width: 100px;
    scroll-marker-group: after;
    counter-reset: t;
  }

  #scroller div {
    width: 100px;
    height: 100px;
    background-color: blue;
    margin: 5px;
    counter-increment: t;
  }

  #scroller div::scroll-marker {
    content: "t" counter(t);
    color: red;
  }

  #scroller div::scroll-marker:target-current {
    color: green;
  }

  a {
    color: red;
  }

  a:target-current {
    color: green;
  }
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “scroll-target-group”.",
      "severity": "Error"
    },
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
