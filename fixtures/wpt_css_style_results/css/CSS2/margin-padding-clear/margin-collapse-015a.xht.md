# css/CSS2/margin-padding-clear/margin-collapse-015a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-015a.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  font: 20px/1 Ahem;
  width: 5em;
  }

  div#wrapper
  {
  background-image: url("support/margin-collapse-2em-space.png");
  height: 4em;
  }

  div#inline-block
  {
  display: inline-block;
  margin-bottom: 2.5em;
  vertical-align: bottom;
  }

  div#block-child-of-inline-block
  {
  display: block;
  margin-bottom: 3.5em;
  margin-top: 0.5em;
  }

  div#wrapper div {background-color: green;}

  div#inline-block, div#block-child-of-inline-block {height: 0.5em;}

  div#following-sibling-after-inline-block {height: 1em;}
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
