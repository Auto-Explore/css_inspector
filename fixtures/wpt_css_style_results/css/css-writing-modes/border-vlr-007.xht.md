# css/css-writing-modes/border-vlr-007.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/border-vlr-007.xht"
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
      background-color: transparent;
      height: 50px; /* necessary, otherwise inner blocks must grow as tall as the height of viewport */
    }

  .foo
    {
      border-bottom: blue solid 5px;
      border-left: blue solid 100px;
      border-right: blue solid 50px;
      border-top: blue solid 20px;
      writing-mode: vertical-lr;
    }

  .bar
    {
      border-bottom: blue solid 20px;
      border-left: blue solid 50px;
      border-right: blue solid 100px;
      border-top: blue solid 5px;
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
