# css/css-writing-modes/text-orientation-016.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-orientation-016.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      border-bottom: blue solid 15px;
      border-top: blue solid 15px;
    }

  div#test
    {
      background-color: blue;
      color: yellow;
      font-family: Ahem;
      font-size: 20px;
      line-height: 2;
      text-orientation: sideways;
      writing-mode: vertical-rl;
    }

  span#p80
    {
      font-size: 4em; /* computes to 80px */
    }

  span#p40
    {
      font-size: 2em; /* computes to 40px */
    }

  span#E10
    {
      font-size: 0.5em; /* computes to 10px */
    }

  span#E20
    {
      font-size: 1em; /* computes to 20px */
    }

  div#reference
    {
      margin-top: 8px;
      width: 200px;
    }

  img
    {
      vertical-align: top;
    }
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
