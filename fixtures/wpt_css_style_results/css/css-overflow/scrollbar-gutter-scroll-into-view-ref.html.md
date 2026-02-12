# css/css-overflow/scrollbar-gutter-scroll-into-view-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scrollbar-gutter-scroll-into-view-ref.html"
}
```

## style[0]

```css

  :root {
    scrollbar-gutter: stable both-edges;
    writing-mode: vertical-lr;
    overflow: hidden;
  }

  body {
    margin: 0;
  }

  #target {
    width: 100px;
    height: 100px;
    border: 4px solid black;
    position: absolute;
    left: 0;
    top: 0px;
    background-color: lightgreen;=
  }
  #bg {
    background-color: khaki;
    width: 200vw;
    height: 400vh;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
