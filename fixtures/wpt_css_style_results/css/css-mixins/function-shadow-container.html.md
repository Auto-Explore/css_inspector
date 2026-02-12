# css/css-mixins/function-shadow-container.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/function-shadow-container.html"
}
```

## style[0]

```css

      @function --b() {
        @container --cont (width = 10px) {
          result: 10px;
        }
      }
      ::part(target) {
        --actual: --b();
        --expected: 10px;
      }
      .container {
        container: --cont / size;
        width: 10px;
        height: 10px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

      @function --b() {
        @container --cont (width = 5px) {
          result: 5px;
        }
        @container --cont (width = 10px) {
          result: 10px;
        }
      }
      ::part(target) {
        --actual: --b();
        --expected: 5px;
      }
      .container {
        container: --cont / size;
        width: 10px;
        height: 10px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css

          @function --b() {
            result: FAIL;
          }
          .container {
            container: --cont / size;
            width: 5px;
            height: 5px;
          }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[3]

```css

      @function --b() {
        @container --cont (width = 5px) {
          result: 5px;
        }
        @container --cont (width = 10px) {
          result: 10px;
        }
      }
      .container {
        container: --cont / size;
        width: 10px;
        height: 10px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[4]

```css

          @function --c() {
            @container --cont (width = 5px) {
              result: 5px;
            }
            @container --cont (width = 10px) {
              result: 10px;
            }
          }
          .container {
            container: --cont / size;
            width: 5px;
            height: 5px;
          }
          ::slotted(#target) {
            --actual: --b() --c();
            --expected: 5px 5px;
          }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
