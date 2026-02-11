# css/css-conditional/container-queries/display-contents.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/display-contents.html"
}
```

## style[0]

```css

  .container {
    container-type: inline-size;
    width: 30px;
    height: 30px;
    background: tomato;
  }
  .big {
    width: 50px;
    height: 50px;
    background: skyblue;
  }
  .contents {
    display: contents;
  }

  @container (width: 30px) {
    .target { --x:30; }
  }

  @container (width: 50px) {
    .target { --x:50; }
  }

  main {
    display: flex;
    flex-wrap: wrap;
  }

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
