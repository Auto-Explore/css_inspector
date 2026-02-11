# css/cssom-view/scrollParent-shadow-tree.html

```json
{
  "format_version": 3,
  "file": "css/cssom-view/scrollParent-shadow-tree.html"
}
```

## style[0]

```css

  .scroller {
    overflow: scroll;
    border: 1px solid blue;
  }
  .scroller {
    height: 150px;
  }
  .target {
    background: repeating-linear-gradient(30deg, white 0px, white 30px, green 30px, green 60px);
    height: 1000px;
  }
  .spacer {
    background: repeating-linear-gradient(-30deg, white 0px, white 30px, blue 30px, blue 60px);
    height: 1000px;
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

## style[1]

```css

          .shadow-scroller {
            overflow: scroll;
            height: 50px;
            border: 1px solid black;
          }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
