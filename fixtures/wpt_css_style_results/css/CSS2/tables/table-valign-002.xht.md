# css/CSS2/tables/table-valign-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/table-valign-002.xht"
}
```

## style[0]

```css
<![CDATA[
    .control {
       width: 20px;
       height: 30px;
       border: solid blue 35px;
       border-style: solid none;
       background: yellow;
    }
    table, .control {
      float: left;
      border-spacing: 0;
    }
    td {
      background: blue;
      padding: 0;
    }
    .content {
      width: 20px;
      height: 30px;
      background: yellow;
      color: yellow;
    }
    .one { height: 100px; }

    .middle td { vertical-align: middle; }
    .baseline td { vertical-align: baseline; }
    .baseline .two .content { margin-top: 35px; }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
