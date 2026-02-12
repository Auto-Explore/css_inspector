# css/css-flexbox/flexbox-align-self-vert-004-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-vert-004-ref.xhtml"
}
```

## style[0]

```css

      .flexbox {
        border: 1px dashed blue;
        width: 4px;
        float: left;
        font-family: sans-serif;
        font-size: 10px;
        margin-left: 80px;
      }

      .flex-start, .flex-end, .center, .baseline, .stretch {
        clear: both;
        margin:       1px 2px 3px 4px;
        border-width: 2px 3px 4px 5px;
        padding:      3px 4px 5px 6px;
        border-style: dotted;
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
