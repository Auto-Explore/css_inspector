# css/css-flexbox/flexbox-align-self-vert-001-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-vert-001-ref.xhtml"
}
```

## style[0]

```css

      .flexbox {
        border: 1px solid blue;
        width: 200px;
        font-size: 10px;
      }

      div.big {
        font-size: 20px;
        width: 50px;
      }

      .flexbox > * {
        clear: both;
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
        float: right;
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
   
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
