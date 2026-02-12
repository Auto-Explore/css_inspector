# css/css-flexbox/flexbox-align-self-vert-rtl-001-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-vert-rtl-001-ref.xhtml"
}
```

## style[0]

```css

      .flexbox {
        border: 1px solid blue;
        width: 200px;
        direction: rtl;
        font-family: sans-serif;
        font-size: 10px;
      }

      div.big {
        font-size: 20px;
        width: 50px;
      }

      div {
        clear: both;
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
      .center {
        background: lightblue;
        margin: auto;
      }
      .baseline {
        background: teal;
        float: left;
      }
      .stretch {
        background: pink;
        width: 100%;
      }
      .auto {
        background: yellow;
        margin: auto;
      }
      .unspecified {
        background: lightgreen;
        margin: auto;
      }
      .initial {
        background: aqua;
        margin: auto;
      }
      .inherit {
        background: violet;
        float: left;
      }

      <!-- We center shrinkwrapped text by putting it into an inline-block, and
           then wrapping that inline-block in a helper-div that has
           "text-align:center" set. -->
      .centerParent {
        text-align: center;
      }
      .centerParent > * {
        display: inline-block;
        text-align: left; /* Keep parent's centering from tweaking my text */
      }

      .baselineParent {
        float: right;
      }
   
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
