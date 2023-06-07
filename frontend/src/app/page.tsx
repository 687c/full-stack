'use client'

import Image from 'next/image'
import useSWR from 'swr'
import { useEffect, useState } from 'react'

//@ts-ignore
const fetcher = url => fetch(url).then(r => r.json())

export default function Home() {
  const [data, setData] = useState();
  const [searchQuery, setSearchQuery] = useState("");

  const handleSearchQueryChange = (event: any) => {
    const { target: { value } } = event;
    setSearchQuery(value);
  };

  //@ts-ignore
  function boldenSimilar(normal, highlighted) {
    const maxLength = Math.max(normal.length, highlighted.length);
    const highlightedWord = [];

    for (let i = 0; i < maxLength; i++) {
      if (normal[i] === highlighted[i]) {
        highlightedWord.push(<strong key={i}>{normal[i]}</strong>);
      } else {
        highlightedWord.push(normal[i]);
      }
    }

    return highlightedWord;
  }

  //debouncing
  useEffect(() => {
    const timer = setTimeout(() => {
      if (searchQuery === "") {
        setData(undefined);
        return;
      }

      fetch(`http://127.0.0.1:8080/search?query=${searchQuery}`)
        .then(res => res.json())
        .then(json => {
          setData(json)
        })

    }, 500);

    return () => clearTimeout(timer);

  }, [searchQuery]);

  async function joinArr(arr: string[]) {
    return arr.join('')
  }

  return (
    <main className="flex min-h-screen flex-col items-center  p-24">
      <div>
        <h1 className="text-gray-900 text-4xl font-extrabold" >
          search mpn
        </h1>
      </div>
      <div>
        <input
          className="border border-gray-300 mt-20 mb-3 height-20 text-center outline-none h-10"
          type="text"
          value={searchQuery}
          onChange={handleSearchQueryChange}
          placeholder="enter mpn "
        />
        <div>

          {
            //@ts-ignore
            data?.results.map((mpn, index) => (
              <div
                style={{
                  backgroundColor: index % 2 === 0 ? 'lightgrey' : 'transparent',
                }}
                key={index}>
                <div className='p-2' >
                  {
                    boldenSimilar(mpn.normal, mpn.highlighted)
                  }
                </div>
              </div>
            ))
          }
        </div>
      </div>
    </main>
  )
}
