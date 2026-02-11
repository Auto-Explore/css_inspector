# css/css-transforms/individual-transform/individual-transform-1.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/individual-transform/individual-transform-1.html"
}
```

## style[0]

```css

      div {
        position: fixed;
        width: 100px;
        height: 100px;
        transform-origin: 0 0;
        border-style: solid;
        border-width: 10px 0px 10px 0px;
        border-color: lime;
      }
      .row_1 {
        top: 100px;
      }
      .scale_1{
        left: 100px;
        width: 50px;
        height: 100px;
        /* test 'scale: <number>' */
        scale: 2;
      }
      .translate_1 {
        left: 150px;
        /* test 'translate: <length-percentage>' */
        translate: 150px;
      }
      .rotate_1 {
        left: 450px;
        transform-origin: 50% 50%;
        /* test 'rotate: <angle>' */
        rotate: 90deg;
      }

      .row_2 {
        top: 250px;
      }
      .scale_2{
        left: 100px;
        width: 50px;
        height: 50px;
        /* test 'scale: <number>{2}'' */
        scale: 2 2;
      }
      .translate_2 {
        left: 150px;
        top: 200px;
        /* test 'translate: <length-percentage><length-percentage>' */
        translate: 150px 50px;
      }
      .rotate_2 {
        left: 450px;
        transform-origin: 50% 50%;
        /* test 'rotate: axis <angle>'*/
        rotate: x 45deg;
      }
      .row_3 {
        transform: perspective(500px);
        top: 400px;
      }
      .scale_3{
        left: 100px;
        width: 50px;
        height: 50px;
        /* test 'scale: <number>{3}'' */
        scale: 2 2 2;
      }
      .translate_3 {
        left: 150px;
        /* test 'translate: <length-percentage><length>' */
        translate: 150px 10px 10px;
      }
      .rotate_3 {
        left: 450px;
        transform-origin: 50% 50%;
        /* test 'rotate: <number>{3}<angle>'*/
        rotate: 0 1 0 45deg;
      }
    
```

```json
{
  "errors": 10,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “scale”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “translate”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “rotate”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “scale”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “translate”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “rotate”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
