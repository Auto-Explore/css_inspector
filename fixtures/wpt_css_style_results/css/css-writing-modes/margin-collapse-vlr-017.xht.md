# css/css-writing-modes/margin-collapse-vlr-017.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/margin-collapse-vlr-017.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      font: 25px/1em Ahem;
      height: 4em;
    }

  div#wrapper
    {
      background-color: green;
      width: 8em;
      writing-mode: vertical-lr;
    }

  div#overflow
    {
      background-color: white;
      margin-left: 4em; /* block-start margin of overflowed block */
      overflow: hidden;
      width: 4em;
    }

  div#nested
    {
      background-color: red;
      margin-left: 4em; /* block-start margin of sub-block */
      width: 4em;
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
