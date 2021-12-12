import tw from 'twin.macro'

const Backdrop = tw.div`
	flex items-center justify-center
	min-h-screen text-white bg-transparent
`

const range = (n: number) => Array.from({length: n}).map((_, n) => n)

function App() {
  return (
    <Backdrop data-tauri-drag-region>
      <div tw="grid grid-cols-9 gap-3" data-tauri-drag-region>
        {range(81).map((n) => (
          <div
            key={n}
            tw="bg-white w-10 h-10 rounded-lg cursor-pointer hover:bg-violet-200"
            css={{
              background:
                'radial-gradient(circle, hsla(195, 94%, 67%, 1) 45%, hsla(0, 0%, 88%, 1) 100%)',

              '&:hover': {
                background: 'hsla(206, 94%, 67%, 1)',
              },
            }}
          />
        ))}
      </div>
    </Backdrop>
  )
}

export default App
