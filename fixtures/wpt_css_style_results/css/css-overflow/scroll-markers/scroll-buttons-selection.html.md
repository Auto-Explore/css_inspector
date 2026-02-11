# css/css-overflow/scroll-markers/scroll-buttons-selection.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-buttons-selection.html"
}
```

## style[0]

```css

    * {
      margin: 0;
      padding: 0;
    }

    #scroller {
        width: 600px;
        height: 300px;
        overflow: scroll;
        scroll-marker-group: before;
    }

    #scroller div {
        width: 500px;
        height: 300px;
        margin-bottom: 20px;
        background: green;
    }

    #scroller::scroll-marker-group {
        border: 3px solid black;
        padding: 5px;
        display: block;
    }

    #scroller div::scroll-marker {
        content: "";
        width: 50px;
        height: 50px;
        background-color: blue;
        display: inline-block;
    }

    #scroller #first::scroll-marker {
        background-color: purple;
        margin-right: 4px;
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
