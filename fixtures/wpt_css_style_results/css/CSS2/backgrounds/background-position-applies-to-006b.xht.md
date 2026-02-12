# css/CSS2/backgrounds/background-position-applies-to-006b.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-position-applies-to-006b.xht"
}
```

## style[0]

```css
<![CDATA[
  table {border-spacing: 0px;}

  col#tested-col
  {
  background-image: url("/css/support/60x60-red.png");
  background-position: bottom right;
  background-repeat: no-repeat;
  }

  td {padding: 0px;}

  td#top-left {border-top: transparent solid 60px;}

  td#top-right {border-right: transparent solid 60px;}

  td#bottom-left
  {
  border-bottom: transparent solid 60px;
  border-left: transparent solid 60px;
  }

  td#green-overlapping {border-bottom: green solid 60px;}
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
