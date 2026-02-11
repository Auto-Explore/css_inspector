# css/css-scrollbars/scrollbar-width-016.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-width-016.html"
}
```

## style[0]

```css

  .container {
    overflow: auto;
    height: 200px;
    width: 200px;
    margin: 1px;
    padding: 0px;
    border: none;
    background: deepskyblue;
  }

  .content {
    height: 300px;
    width: 100%;
    background: lightsalmon;
  }

  .container.auto {
    scrollbar-width: auto;
  }

  .container.auto::-webkit-scrollbar {
    display: none;
  }

  /* This is so that browsers that don't implement the WebKit prefix still pass the test */
  @supports not selector(::-webkit-scrollbar) {
    .container.auto {
      overflow: hidden;
    }
  }

  .container.thin {
    scrollbar-width: thin;
  }

  .container.thin::-webkit-scrollbar {
    display: none;
  }

  .container.none {
    scrollbar-width: none;
  }

  .container.none::-webkit-scrollbar {
    width: 20px;
    background-color: lightgray;
  }
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
