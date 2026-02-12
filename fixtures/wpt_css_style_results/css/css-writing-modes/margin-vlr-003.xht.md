# css/css-writing-modes/margin-vlr-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/margin-vlr-003.xht"
}
```

## style[0]

```css
<![CDATA[
  .outer
    {
      background-color: blue;
      border: blue solid 3px;
      width: 200px;
    }

  hr
    {
      background-color: transparent;
      border: transparent none 0px;
      height: 3px;
      margin: 0.5em auto;
    }

  .inner
    {
      background-color: yellow;
      height: 50px;
      width: 50px;
    }

  .foo
    {
      margin-bottom: 5px;
      margin-left: 100px;
      margin-right: 50px;
      margin-top: 20px;
      writing-mode: vertical-lr;
    }

  .bar
    {
      margin-bottom: 20px;
      margin-left: 50px;
      margin-right: 100px;
      margin-top: 5px;
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
