import tw from "twin.macro"

import { invoke, emit } from "@tauri-apps/api"

const Backdrop = tw.div`
	flex items-center justify-center
	min-h-screen text-white bg-transparent
`

const rows = 9
const range = (n: number) => Array.from({ length: n }).map((_, n) => n)

const toXY = (n: number): [x: number, y: number] => [
  n % rows,
  Math.floor(n / rows)
]

function App() {
  return (
    <Backdrop data-tauri-drag-region>
      <div tw="grid grid-cols-9 gap-3" data-tauri-drag-region="">
        {range(rows * rows).map((n) => {
          const [x, y] = toXY(n)

          return (
            <div
              onClick={() => emit("keypress", { pos: 20 })}
              key={n}
              tw="flex items-center justify-center bg-white w-8 h-8 rounded-lg cursor-pointer hover:bg-violet-200"
              css={{
                background:
                  "radial-gradient(circle, hsla(195, 94%, 67%, 1) 45%, hsla(0, 0%, 88%, 1) 100%)",

                "&:hover": {
                  background: "hsla(206, 94%, 67%, 1)"
                },

                ...((x === 8 || y === 0) && {
                  background: "pink",

                  "&:hover": {
                    background: "darkpink"
                  }
                }),

                ...(x === 8 && y === 0 && { background: "transparent" })
              }}
            />
          )
        })}
      </div>
    </Backdrop>
  )
}

export default App
