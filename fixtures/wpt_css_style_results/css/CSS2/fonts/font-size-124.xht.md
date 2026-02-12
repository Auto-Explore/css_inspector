# css/CSS2/fonts/font-size-124.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/fonts/font-size-124.xht"
}
```

## style[0]

```css
<![CDATA[
  div#overlapped-red
  {
  background-color: red;
  height: 100px;
  width: 100px;
  }

  div#grand-parent {font: 25px/1 Ahem;}
  /*
  To download Ahem font:
  http://www.w3.org/Style/CSS/Test/Fonts/Ahem/
  */

  div#parent {font-size: 5ex;}
  /* The Ahem font has an x-height of 0.8em. */

  div#overlapping-green
  {
  background-color: green;
  bottom: 100px;
  font-family: serif;
  position: relative;
  width: 100px;
  }

  /*
  In this test, the inherited computed font-size of
  div#overlapping-green should be 100px with a line box
  height of 100px.
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
