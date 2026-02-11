# css/css-contain/contain-paint-ignored-cases-no-principal-box-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-ignored-cases-no-principal-box-001-ref.html"
}
```

## style[0]

```css

    div {
      position: relative;
      width: 100px;
    }
    #div1,
    #div3 {
      background-color: #cfc;
      height: 100px;
    }
    #div1 {
      z-index: 5;
    }
    #div2 {
      display: contents;
      background-color: #fdd;
      height: 100px;
      top: -20px;
    }
    #div2_1 {
      background-color: #ffc;
      z-index: 6;
      top: -10px;
      height: 100px;
    }
    #div2_2 {
      z-index: 3;
      position: absolute;
      top: -15px;
      width: 40px;
      height: 300px;
      background-color: #ddf;
    }
    #div3 {
      z-index: 2;
      top: -50px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
