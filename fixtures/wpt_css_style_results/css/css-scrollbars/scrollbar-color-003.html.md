# css/css-scrollbars/scrollbar-color-003.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-color-003.html"
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
    scrollbar-color: auto;
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

  .container.themed {
    scrollbar-color: yellow blue;
  }

  .container.themed::-webkit-scrollbar {
    display: none;
  }
```

```json
{
  "errors": 5,
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
      "message": "Invalid value for property “scrollbar-color”.",
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
