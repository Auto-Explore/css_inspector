# css/css-writing-modes/padding-vlr-005.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/padding-vlr-005.xht"
}
```

## style[0]

```css
<![CDATA[
  .outer
    {
      border: blue solid 3px;
      width: 200px;
    }

  hr
    {
      background-color: blue;
      border: transparent none 0px;
      height: 9px;
      margin: 0px;
    }

  .inner
    {
      background-color: blue;
      height: 50px; /* necessary, otherwise inner blocks will grow as tall as the height of viewport */
    }

  .foo
    {
      padding-bottom: 5px;
      padding-left: 100px;
      padding-right: 50px;
      padding-top: 20px;
      writing-mode: vertical-lr;
    }

  .bar
    {
      padding-bottom: 20px;
      padding-left: 50px;
      padding-right: 100px;
      padding-top: 5px;
      writing-mode: vertical-lr;
    }

  div#reference
    {
      margin-top: 1em;
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
