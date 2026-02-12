# css/css-flexbox/flexbox-align-self-vert-rtl-002-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-vert-rtl-002-ref.xhtml"
}
```

## style[0]

```css

      .flexbox {
        border: 1px dashed blue;
        width: 200px;
        float: left;
        direction: rtl;
        font-family: sans-serif;
        font-size: 10px;
      }

      .flex-start, .flex-end, .center, .baseline, .stretch {
        clear: both;
        margin:       1px 2px 3px 4px;
        border-width: 2px 3px 4px 5px;
        padding:      3px 4px 5px 6px;
        border-style: dotted;
      }

      div.big {
        font-size: 20px;
        width: 50px;
      }

      /* Classes for each of the various align-self values */
      .flex-start {
        background: lime;
        float: right;
      }
      .flex-end {
        background: orange;
        float: left;
      }
      
      .centerParent {
        text-align: center;
      }
      .center {
        background: lightblue;
        display: inline-block;
        text-align: right; /* Keep parent's centering from tweaking my text */
      }
      .baselineParent {
        float: right;
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
