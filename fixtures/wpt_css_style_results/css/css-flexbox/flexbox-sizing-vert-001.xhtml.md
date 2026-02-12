# css/css-flexbox/flexbox-sizing-vert-001.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-sizing-vert-001.xhtml"
}
```

## style[0]

```css

      div { width: 10px; }
      div.flexbox {
        float: left;
        border: 1px dashed blue;
        font-size: 10px;
        display: flex;
        flex-direction: column;
        margin-right: 2px;
      }
      div.a {
        flex: 1 20px;
        max-height: 60px;
        background: lightgreen;
      }
      div.b {
        flex: 1 20px;
        min-height: 40px;
        max-height: 60px;
        background: purple;
      }
      div.c {
        flex: 1 40px;
        min-height: 10px;
        max-height: 60px;
        background: orange;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
