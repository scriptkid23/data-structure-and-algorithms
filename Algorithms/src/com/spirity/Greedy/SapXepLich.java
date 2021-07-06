package com.spirity.Greedy;

import java.util.ArrayList;

public class SapXepLich {
    public static void main(String[] arg){
        ArrayList<CongViec> listCV = new ArrayList<>();
        listCV.add(new CongViec("A",0,6));
        listCV.add(new CongViec("B",1,4));
        listCV.add(new CongViec("C",3,5));
        listCV.add(new CongViec("D",3,8));
        listCV.add(new CongViec("E",4,7));
        listCV.add(new CongViec("F",5,9));
        listCV.add(new CongViec("G",6,10));
        listCV.add(new CongViec("H",8,11));
        for(int i = 0; i < listCV.size() - 1;i++){
            for(int j = i+1; j < listCV.size();j++)
            if(listCV.get(i).end > listCV.get(j).end){
                CongViec temp = listCV.get(i);
                listCV.set(i,listCV.get(j));
                listCV.set(j,temp);

            }
        }
        ArrayList<CongViec> dapan = new ArrayList<>();
        dapan.add(listCV.get(0));
        for(int i = 0; i < listCV.size();i++){
            if(listCV.get(i).start  >= dapan.get(dapan.size() - 1).end){
                dapan.add(listCV.get(i));
            }
        }
        System.out.println(dapan.size());
    }
}
class CongViec{
    public String name;
    public int start;
    public int end;
    public CongViec(String name, int start, int end){
        this.start = start;
        this.end = end;
        this.name = name;
    }
}
