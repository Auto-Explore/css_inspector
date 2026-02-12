# css/CSS2/backgrounds/background-position-applies-to-007a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-position-applies-to-007a.xht"
}
```

## style[0]

```css
<![CDATA[
  div#table {display: table;}

  div#tbody {display: table-row-group;}

  div.tr {display: table-row;}

  div.td
  {
  display: table-cell;
  height: 1in;
  width: 1in;
  }

  div#tested-cell
  {
  background-image: url('support/blue96x96.png');
  background-position: bottom right;
  background-repeat: no-repeat;
  border-bottom: white solid 96px;
  /*
  The goal/challenge in this test is to verify
  that such white border-bottom does not
  cover the background-image.
  The background-image should "start" being
  painted at bottom right corner of
  padding-box of element and not "start"
  being painted at bottom right corner of
  its border-box.
  */
  display: table-cell;
  height: 2in;
  width: 1in;
  }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
