# css/CSS2/visuren/inherit-static-offset-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visuren/inherit-static-offset-003.xht"
}
```

## style[0]

```css
<![CDATA[
  div#parent
  {
  background: red url("support/pattern-gg-gr-100x100.png") no-repeat;
  height: 100px;
  left: 3.125em;
  position: static;
  top: 3.125em;
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
  font-size: 6.25em;
  left: inherit;
  position: relative;
  top: inherit;
  /*
  What is inherited is a computed value
  (which is, in this test, 3.125em mult by 16px == 50px),
  not the specified value (6.25em) of parent.
  */
  width: 50px;
  }

  /*
  In this test, left and top offset values will be inherited
  */
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
