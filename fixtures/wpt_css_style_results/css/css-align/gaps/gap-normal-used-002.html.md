# css/css-align/gaps/gap-normal-used-002.html

```json
{
  "format_version": 3,
  "file": "css/css-align/gaps/gap-normal-used-002.html"
}
```

## style[0]

```css

#flex {
  colum-gap: normal;
  row-gap: normal;
  display: flex;
  flex-flow: wrap;
  max-width: 145px; /* more than 100, less than 150, to force wrapping and get 2 items per line*/

  position: absolute;
}
#flex * {
  width: 50px;
  height: 50px;
  background: green
}
#red {
  width: 100px;
  height: 100px;
  background: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
