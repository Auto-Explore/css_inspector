# css/css-flexbox/flex-grow-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-grow-001.xht"
}
```

## style[0]

```css
<![CDATA[
   #container {
     width: 240px;
     height: 60px;
     border: 1px solid #000;
     display: flex;
   }
   #container > div {
     width: 30px;
     height: 100%;
   }
   #child_1 {
     flex-grow: 0;
     background-color: green;
   }
   #child_2 {
     flex-grow: 1;
     background-color: blue;
   }
   #child_3 {
     flex-grow: 2;
     background-color: gray;
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
