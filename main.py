from fastapi import FastAPI, HTTPException
from fastapi.responses import StreamingResponse
from pydantic import BaseModel
from typing import List, Dict, Any
import json
import asyncio

app = FastAPI()

class UIMessage(BaseModel):
    role: str
    parts: List[Dict[str, Any]]

class ChatRequest(BaseModel):
    chat_id: str
    model: str
    messages: List[UIMessage]

@app.post("/api/agent/chat")
async def chat_with_agent(request: ChatRequest):
    """
    模拟AI Agent的聊天接口
    返回兼容toUIMessageStream()格式的流式响应
    """
    
    # 静态回复字符串
    static_response = "这是一个来自FastAPI Agent的静态回复。我可以处理您的请求并提供帮助。"
    
    # 将静态字符串转换为流式响应
    async def generate_stream():
        # 模拟流式输出，将静态字符串分块发送
        chunk_size = 5  # 每次发送5个字符
        for i in range(0, len(static_response), chunk_size):
            chunk = static_response[i:i + chunk_size]
            
            # 构建符合toUIMessageStream()格式的NDJSON数据
            if i == 0:
                # 第一个chunk，发送text-start
                yield f'{{"type":"text-start","id":"agent-response"}}\n'
            
            # 发送text-delta
            yield f'{{"type":"text-delta","id":"agent-response","delta":"{chunk}"}}\n'
            
            # 模拟网络延迟
            await asyncio.sleep(0.1)
        
        # 最后一个chunk，发送text-end
        yield f'{{"type":"text-end","id":"agent-response"}}\n'
    
    return StreamingResponse(
        generate_stream(),
        media_type="application/x-ndjson",
        headers={
            "Content-Type": "application/x-ndjson",
            "Cache-Control": "no-cache",
            "Connection": "keep-alive"
        }
    )


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="127.0.0.1", port=5000)
