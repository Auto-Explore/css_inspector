# css/CSS2/visuren/inherit-static-offset-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visuren/inherit-static-offset-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div#parent
  {
  background: red url("support/pattern-gg-gr-100x100.png") no-repeat;
  height: 100px;
  left: 50px;
  position: static;
  top: 50px;
  /*
  The 'top', 'right', 'bottom', and 'left' properties
  do not apply on a statically positioned element
  */
  width: 100px;
  }

  div#child
  {
  background-color: green;
  height: 50px;
  left: inherit;
  position: relative;
  top: inherit;
  width: 50px;
  }

  /*
  In this test, left and top offset values will be inherited
  */
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
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
