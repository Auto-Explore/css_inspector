# css/CSS2/floats-clear/margin-collapse-clear-017.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/margin-collapse-clear-017.xht"
}
```

## style[0]

```css
<![CDATA[
  body
  {
  background: white url("support/ruler-v-100px-200px-300px.png") no-repeat;
  margin: 34px 8px 8px 55px;
  /*
    34px : body's top margin collapsing with p's top margin: max(34px, 16px)
  + 20px : p's 1st line box height
  + 20px : p's 2nd line box height
  + 16px : p's bottom margin
  + 10px : height of topmost green bar
  ======
   100px
  */
  }

  p
  {
  font-size: 16px;
  line-height: 20px;
  margin: 1em 8px;
  }

  div#parent-block {margin-bottom: 100px;}

  div.vertical-gap-separator
  {
  background-color: green;
  height: 10px;
  }

  div#element-without-clearance
  {
  clear: both;
  margin-top: 100px;
  }

  /*
  In this test, div#element-without-clearance
  is what the spec refers to as
  'collapsed through' box
  http://www.w3.org/TR/CSS21/box.html#collapsed-through
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
