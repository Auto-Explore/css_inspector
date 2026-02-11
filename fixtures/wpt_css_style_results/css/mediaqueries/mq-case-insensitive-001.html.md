# css/mediaqueries/mq-case-insensitive-001.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-case-insensitive-001.html"
}
```

## style[0]

```css


  div {
   width: 100px;
   height: 100px;
  }

  @media all and (height) and (min-width:0) and (orientation:landscape) {
   div { background-color: red; }
  }
  @media all and (height) and (min-width:0) and (orientation:portrait) {
   div { background-color: red; }
  }

  @MeDIa aLL and (Height) and (mIN-Width:0cM) and (orienTAtion:LandScape) {
   div { background-color: green; }
  }
  @MeDIa All and (heiGHt) and (Min-widtH:0MM) and (Orientation:porTrait) {
   div { background-color: green; }
  }

  /* In some languages Non-ASCII 'İ' (Latin capital I with dot above) may be
     lowercased to ASCII 'i'; This would make "heİght" compare the same as
     "height", which would be incorrect. */
  @media all and (heİght) {
   div { background-color: red; }
  }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
