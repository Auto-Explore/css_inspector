# css/css-flexbox/flexbox-align-self-vert-002-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-vert-002-ref.xhtml"
}
```

## style[0]

```css

      .flexbox {
        border: 1px dashed blue;
        width: 200px;
        float: left;
        font-size: 10px;
      }

      .flex-start, .flex-end, .center, .baseline, .stretch,
      .self-start, .self-end {
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
        float: left;
      }
      .flex-end {
        background: orange;
        float: right;
      }
      <!-- We center shrinkwrapped text by putting it into an inline-block, and
           then wrapping that inline-block in a helper-div that has
           "text-align:center" set. -->
      .centerParent {
        text-align: center;
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
      .self-start {
        background: yellow;
        float: right;
        text-align: right;
      }
      .self-end {
        background: purple;
        float: left;
        text-align: right;
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
