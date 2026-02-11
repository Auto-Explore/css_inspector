# css/css-conditional/container-queries/display-none.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/display-none.html"
}
```

## style[0]

```css

  .container {
    container-type: size;
    width: 30px;
    height: 30px;
    background: tomato;
  }
  .small {
    width: 10px;
    height: 10px;
  }
  .big {
    width: 50px;
    height: 50px;
    background: skyblue;
  }
  .auto {
    width: auto;
  }
  .none {
    display: none;
  }
  .pseudo::before {
    content: "foo";
  }
  .pseudo_none::before {
    content: "foo";
    display: none;
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
