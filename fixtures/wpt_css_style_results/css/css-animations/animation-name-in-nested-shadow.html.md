# css/css-animations/animation-name-in-nested-shadow.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-name-in-nested-shadow.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  @keyframes doc {
    from, to { background-color: lightgreen }
  }

  .anim {
    width: 100px;
    height: 100px;
    background-color: red;
    animation-duration: 1s;
    animation-fill-mode: both;
  }

  #doc_anim_doc { animation-name: doc; }
  #doc_anim_outer { animation-name: outer; }
  #doc_anim_inner { animation-name: inner; }

  #outer_host {
    position: absolute;
    left: 100px;
    top: 0;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

      @keyframes outer {
        from, to { background-color: green }
      }

      .anim {
        width: 100px;
        height: 100px;
        background-color: red;
        animation-duration: 1s;
        animation-fill-mode: both;
      }

      #outer_anim_doc { animation-name: doc; }
      #outer_anim_outer { animation-name: outer; }
      #outer_anim_inner { animation-name: inner; }

      #inner_host {
        position: absolute;
        left: 100px;
        top: 0;
      }

    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

          @keyframes inner {
            from, to { background-color: darkgreen }
          }

          .anim {
            width: 100px;
            height: 100px;
            background-color: red;
            animation-duration: 1s;
            animation-fill-mode: both;
          }

          #inner_anim_doc { animation-name: doc; }
          #inner_anim_outer { animation-name: outer; }
          #inner_anim_inner { animation-name: inner; }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
