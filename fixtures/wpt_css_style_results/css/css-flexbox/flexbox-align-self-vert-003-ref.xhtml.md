# css/css-flexbox/flexbox-align-self-vert-003-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-vert-003-ref.xhtml"
}
```

## style[0]

```css

      .flexbox {
        border: 1px dashed blue;
        width: 4px;
        font-family: sans-serif;
        font-size: 10px;
        margin-left: 80px;
      }

      div.big {
        font-size: 18px;
        width: 50px;
      }

      /* Classes for each of the various align-self values */
      .flex-start {
        background: lime;
        float: left;
      }
      .flex-end {
        background: orange;
        float: right;
      }
      
      .centerParent {
        text-align: center;
        width: 100px;
        margin-left: -48px;
      }
      .center {
        background: lightblue;
        display: inline-block;
        text-align: left; /* Keep parent's centering from tweaking my text */
      }
      .baseline {
        background: teal;
        float: left;
      }
      .stretch {
        background: pink;
      }
      .clearFloats { clear: both }
   
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
